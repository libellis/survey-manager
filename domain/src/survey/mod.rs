pub mod choice;
pub use choice::*;

pub mod question;
pub use question::*;

pub mod events;
pub use events::*;

use survey_manager_commands::*;

use crate::value_objects::{Title, QuestionType, ContentType, Author, Description, Category};
use uuid::Uuid;
use domain_patterns::models::{Entity, AggregateRoot};
use chrono::Utc;
use std::convert::TryFrom;
use crate::errors::Result;
use crate::errors::Error;
use crate::errors::ErrorKind::ResourceNotFound;

#[derive(Entity)]
pub struct Survey {
    id: Uuid,
    version: u64,
    author: Author,
    title: Title,
    description: Description,
    // TODO: Change into a nice timestamp.
    created_on: i64,
    category: Category,
    questions: Vec<Question>,
}

impl Survey {
    pub fn new(
        new_survey: &CreateSurveyCommand,
    ) -> Result<Survey> {
        Ok(Survey {
            id: Uuid::new_v4(),
            version: 0,
            author: Author::try_from(new_survey.author.clone())?,
            title: Title::try_from(new_survey.title.clone())?,
            description: Description::try_from(new_survey.description.clone())?,
            created_on: Utc::now().timestamp(),
            category: Category::try_from(new_survey.category.clone())?,
            questions: Self::create_questions(new_survey.questions.clone())?
        })
    }

    // CONSTRUCTORS FOR CHILD ENTITIES
    fn create_questions(new_questions: Vec<CreateQuestionCommand>) -> Result<Vec<Question>> {
        let q_results = new_questions
            .into_iter()
            .map(|q| { Self::create_question(q) });

        let mut questions: Vec<_> = vec![];

        for q_result in q_results {
            questions.push(q_result?)
        }

        Ok(questions)
    }

    fn create_question(new_question: CreateQuestionCommand) -> Result<Question> {
        Ok(Question {
            id: Uuid::new_v4(),
            version: 0,
            question_type: QuestionType::try_from(new_question.question_type)?,
            title: Title::try_from(new_question.title)?,
            choices: Self::create_choices(new_question.choices)?,
        })
    }

    fn create_choices(new_choices: Vec<CreateChoiceCommand>) -> Result<Vec<Choice>> {
        let c_results = new_choices
            .into_iter()
            .map(|c| { Self::create_choice(c) });

        let mut choices: Vec<_> = vec![];

        for c_result in c_results {
            choices.push(c_result?)
        }

        Ok(choices)
    }

    fn create_choice(new_choice: CreateChoiceCommand) -> Result<Choice> {
        Ok(Choice {
            id: Uuid::new_v4(),
            version: 0,
            // TODO: Update for content translation once we understand embed strings.
            content: None,
            content_type: ContentType::try_from(new_choice.content_type)?,
            title: Title::try_from(new_choice.title)?,
        })
    }

    pub fn belongs_to(&self, author: &String) -> bool {
        &self.author.to_string() == author
    }

    pub fn try_update(&mut self, changeset: SurveyChangeset) -> Result<()> {
        if let Some(new_title) = changeset.title {
            self.change_title(new_title)?;
        }
        if let Some(new_category) = changeset.category {
            self.change_category(new_category)?;
        }
        if let Some(new_desc) = changeset.description {
            self.change_description(new_desc)?;
        }
        if let Some(q_changesets) = changeset.questions {
            self.try_update_questions(q_changesets)?;
        }

        Ok(())
    }

    pub fn change_title(&mut self, new_title: String) -> Result<()> {
        let new_title = Title::try_from(new_title)?;
        self.title = new_title;
        self.version = self.next_version();
        // TODO: Emit a ChangedSurveyTitle event here.
        Ok(())
    }

    pub fn change_category(&mut self, new_category: String) -> Result<()> {
        self.category = Category::try_from(new_category)?;
        self.version = self.next_version();
        // TODO: Emit a ChangedSurveyCategory event here.
        Ok(())
    }

    pub fn change_description(&mut self, new_description: String) -> Result<()> {
        self.description = Description::try_from(new_description)?;
        self.version = self.next_version();
        // TODO: Emit a ChangedSurveyDescription event here.
        Ok(())
    }

    pub fn try_update_questions(&mut self, changesets: Vec<QuestionChangeset>) -> Result<()> {
        for changeset in changesets {
            self.try_update_question(changeset)?;
        }

        Ok(())
    }

    pub fn try_update_question(&mut self, changeset: QuestionChangeset) -> Result<()> {
        let q_id = changeset.id;

        if let Some(new_title) = changeset.title {
            self.change_question_title(&q_id, new_title)?;
        }
        if let Some(new_type) = changeset.question_type {
            self.change_question_type(&q_id, new_type)?;
        }
        if let Some(changesets) = changeset.choices {
            self.try_update_choices(changesets)?;
        }

        Ok(())
    }

    fn find_question(&mut self, q_id: &String) -> Result<&mut Question> {
        // does the question even exist?  Pass back error if not found.
        Ok(
            self.questions
            .iter_mut()
            .find(|q| &q.id() == q_id)
                .ok_or(ResourceNotFound(format!("question with id {}", q_id)))?
        )
    }

    fn find_choice(&mut self, c_id: &String) -> Result<&mut Choice> {
        // does the choice even exist?  Pass back error if not found.
        Ok(
            self.choices_mut()
                .into_iter()
                .find(|c| &c.id() == c_id)
                .ok_or(ResourceNotFound(format!("choice with id {}", c_id)))?
        )
    }

    fn choices_mut(&mut self) -> Vec<&mut Choice> {
        self.questions
            .iter_mut()
            .flat_map(|q| {
                q.choices.iter_mut()
            }).collect()
    }

    pub fn change_question_title(&mut self, q_id: &String, new_title: String) -> Result<()> {
        let question = self.find_question(q_id)?;

        let new_title = Title::try_from(new_title)?;
        question.title = new_title;
        self.version = self.next_version();
        // TODO: Emit a ChangedQuestionTitle event here.
        Ok(())
    }

    pub fn change_question_type(&mut self, q_id: &String, new_type: String) -> Result<()> {
        let question = self.find_question(q_id)?;

        let new_type = QuestionType::try_from(new_type)?;
        question.question_type = new_type;
        self.version = self.next_version();
        // TODO: Emit a ChangedQuestionType event here.
        Ok(())
    }

    pub fn try_update_choices(&mut self, changesets: Vec<ChoiceChangeset>) -> Result<()> {
        for changeset in changesets {
            self.try_update_choice(changeset)?;
        }

        Ok(())
    }

    pub fn try_update_choice(&mut self, changeset: ChoiceChangeset) -> Result<()> {
        let c_id = changeset.id;

        if let Some(new_title) = changeset.title {
            self.change_choice_title(&c_id, new_title)?;
        }
        if let Some(new_type) = changeset.content_type {
            self.change_choice_content_type(&c_id, new_type)?;
        }
        if let Some(new_content) = changeset.content {
            self.change_choice_content(&c_id, new_content)?;
        }

        Ok(())
    }

    pub fn change_choice_title(&mut self, c_id: &String, new_title: String) -> Result<()> {
        let choice = self.find_choice(c_id)?;

        let new_title = Title::try_from(new_title)?;
        choice.title = new_title;
        self.version = self.next_version();
        // TODO: Emit a ChangedChoiceTitle event here.
        Ok(())
    }

    pub fn change_choice_content_type(&mut self, c_id: &String, new_type: String) -> Result<()> {
        let choice = self.find_choice(c_id)?;

        let new_type = ContentType::try_from(new_type)?;
        choice.content_type = new_type;
        self.version = self.next_version();
        // TODO: Emit a ChangedChoiceContentType event here.
        Ok(())
    }

    pub fn change_choice_content(&mut self, c_id: &String, new_content: Option<String>) -> Result<()> {
        let choice = self.find_choice(c_id)?;

        let content = if let Some(c) = new_content {
            // TODO: Obviously replace this once we actually have this figured out.
            Some(Content::Youtube(c))
        } else {
            None
        };

        choice.content = content;
        self.version = self.next_version();
        // TODO: Emit a ChangedChoiceContent event here.
        Ok(())
    }
}

impl AggregateRoot for Survey {
    type Events = SurveyEvents;

    type Error = Error;
}
