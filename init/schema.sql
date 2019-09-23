USE survey-manager;

CREATE TABLE survey (
	id VARCHAR(64) PRIMARY KEY,
	version BIGINT UNSIGNED NOT NULL,
	author VARCHAR(64) NOT NULL,
	title VARCHAR(128) NOT NULL,
	category VARCHAR(64) NOT NULL,
	created_on BIGINT NOT NULL,
	survey_data JSON NOT NULL
);
