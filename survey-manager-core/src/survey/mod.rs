pub mod choice;
pub use choice::*;

pub mod question;
pub use question::*;

pub mod events;
pub use events::*;

use crate::value_objects::{Title, QuestionType, ContentType, Author, Description, Category};
use uuid::Uuid;
use domain_patterns::models::{Entity, AggregateRoot};
use chrono::Utc;
use crate::app_services::commands::*;
use crate::errors::Result;
use crate::errors::Error;
use crate::errors::Error::ResourceNotFound;
use crate::dtos::SurveyDTO;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Entity)]
pub struct Survey {
    id: Uuid,
    version: u64,
    author: Author,
    title: Title,
    description: Description,
    created_on: i64,
    category: Category,
    questions: Vec<Question>,
}

impl AggregateRoot for Survey {
    type Events = SurveyEvents;

    type Error = Error;

    fn version(&self) -> u64 {
        self.version as u64
    }
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
            kind: QuestionType::try_from(new_question.question_type)?,
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
            // TODO: Update for content translation once we understand embed strings.
            content: None,
            content_type: ContentType::try_from(new_choice.content_type)?,
            title: Title::try_from(new_choice.title)?,
        })
    }

    pub fn belongs_to(&self, author: &String) -> bool {
        &self.author.to_string() == author
    }

    pub fn try_update(&mut self, changeset: UpdateSurveyCommand) -> Result<SurveyUpdatedEvent> {
        if let Some(new_title) = &changeset.title {
            self.change_title(new_title)?;
        }
        if let Some(new_category) = &changeset.category {
            self.change_category(new_category)?;
        }
        if let Some(new_desc) = &changeset.description {
            self.change_description(new_desc)?;
        }
        if let Some(q_changesets) = &changeset.questions {
            self.try_update_questions(q_changesets)?;
        }
        // got to here so we succeeded and should version up.
        self.version = self.next_version();
        Ok(self.updated_event(changeset))
    }

    fn change_title(&mut self, new_title: &String) -> Result<()> {
        self.title = Title::try_from(new_title.clone())?;
        Ok(())
    }

    fn change_category(&mut self, new_category: &String) -> Result<()> {
        self.category = Category::try_from(new_category.clone())?;
        Ok(())
    }

    fn change_description(&mut self, new_description: &String) -> Result<()> {
        self.description = Description::try_from(new_description.clone())?;
        Ok(())
    }

    fn try_update_questions(&mut self, changesets: &Vec<UpdateQuestionCommand>) -> Result<()> {
        for changeset in changesets {
            self.try_update_question(changeset)?;
        }

        Ok(())
    }

    fn try_update_question(&mut self, changeset: &UpdateQuestionCommand) -> Result<()> {
        if let Some(new_title) = &changeset.title {
            self.change_question_title(&changeset.id, new_title)?;
        }
        if let Some(new_type) = &changeset.question_type {
            self.change_question_type(&changeset.id, new_type)?;
        }
        if let Some(changesets) = &changeset.choices {
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
                .ok_or(ResourceNotFound { resource: format!("question with id {}", q_id) })?
        )
    }

    fn find_choice(&mut self, c_id: &String) -> Result<&mut Choice> {
        // does the choice even exist?  Pass back error if not found.
        Ok(
            self.choices_mut()
                .into_iter()
                .find(|c| &c.id() == c_id)
                .ok_or(ResourceNotFound { resource: format!("choice with id {}", c_id) })?
        )
    }

    fn choices_mut(&mut self) -> Vec<&mut Choice> {
        self.questions
            .iter_mut()
            .flat_map(|q| {
                q.choices.iter_mut()
            }).collect()
    }

    fn change_question_title(&mut self, q_id: &String, new_title: &String) -> Result<()> {
        let question = self.find_question(q_id)?;
        question.title = Title::try_from(new_title.clone())?;
        Ok(())
    }

    fn change_question_type(&mut self, q_id: &String, new_type: &String) -> Result<()> {
        let question = self.find_question(q_id)?;
        question.kind = QuestionType::try_from(new_type.clone())?;
        Ok(())
    }

    fn try_update_choices(&mut self, changesets: &Vec<UpdateChoiceCommand>) -> Result<()> {
        for changeset in changesets {
            self.try_update_choice(changeset)?;
        }

        Ok(())
    }

    fn try_update_choice(&mut self, changeset: &UpdateChoiceCommand) -> Result<()> {
        if let Some(new_title) = &changeset.title {
            self.change_choice_title(&changeset.id, new_title)?;
        }
        if let Some(new_type) = &changeset.content_type {
            self.change_choice_content_type(&changeset.id, new_type)?;
        }
        if let Some(new_content) = &changeset.content {
            self.change_choice_content(&changeset.id, new_content)?;
        }

        Ok(())
    }

    fn change_choice_title(&mut self, c_id: &String, new_title: &String) -> Result<()> {
        let choice = self.find_choice(c_id)?;
        choice.title = Title::try_from(new_title.clone())?;
        Ok(())
    }

    fn change_choice_content_type(&mut self, c_id: &String, new_type: &String) -> Result<()> {
        let choice = self.find_choice(c_id)?;
        choice.content_type = ContentType::try_from(new_type.clone())?;
        Ok(())
    }

    fn change_choice_content(&mut self, c_id: &String, new_content: &Option<String>) -> Result<()> {
        let choice = self.find_choice(c_id)?;

        let content = if let Some(c) = new_content {
            // TODO: Obviously replace this once we actually have this figured out.
            Some(Content::Youtube(c.clone()))
        } else {
            None
        };

        choice.content = content;
        Ok(())
    }

    // Conversion of cmd to event.  This is being put here for encapsulation reasons.  Only at the end of
    // `try_update` method can we know for sure that the update succeeded in whole, so only the aggregate
    // root (Survey) should be able to create the SurveyUpdatedEvent.  This event should essentially be a
    // changeset (almost identical to the incoming command) so when it's serialized to json it only contains
    // the diff.  At the same time we need data from the aggregate itself, like the version number, so
    // we encapsulate this here to discourage misuse, and allow access to both the cmd data, and direct
    // data from Survey object

    fn updated_event(&self, cmd: UpdateSurveyCommand) -> SurveyUpdatedEvent {
        let questions = if let Some(q) = cmd.questions {
            Some(q.into_iter()
                .map(|q| {
                    QuestionUpdatedEvent {
                        id: q.id,
                        question_type: q.question_type,
                        title: q.title,
                        choices: if let Some(c) = q.choices {
                            Some(c.into_iter()
                                .map(|c| {
                                    ChoiceUpdatedEvent {
                                        id: c.id,
                                        content: c.content,
                                        content_type: c.content_type,
                                        title: c.title
                                    }
                                }).collect())
                        } else {
                            None
                        }
                    }
                }).collect())
        } else {
            None
        };
        SurveyUpdatedEvent {
            id: Uuid::new_v4().to_string(),
            aggregate_id: cmd.id,
            version: self.version(),
            occurred: Utc::now().timestamp(),
            title: cmd.title,
            description: cmd.description,
            category: cmd.category,
            questions,
        }
    }
}

impl From<SurveyDTO> for Survey {
    fn from(dto: SurveyDTO) -> Self {
        let questions: Vec<Question> = dto.questions.into_iter()
            .map(|q| {
                Question::from(q)
            }).collect();
        Survey {
            id: Uuid::from_str(&dto.id).unwrap().clone(),
            version: dto.version,
            author: Author::try_from(dto.author).unwrap(),
            title: Title::try_from(dto.title).unwrap(),
            description: Description::try_from(dto.description).unwrap(),
            created_on: dto.created_on,
            category: Category::try_from(dto.category).unwrap(),
            questions,
        }
    }
}
