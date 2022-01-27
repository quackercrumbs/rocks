-- Your SQL goes here
CREATE TABLE api_response (
	id INTEGER PRIMARY KEY ASC NOT NULL,
	start_date TEXT NOT NULL,
        end_date TEXT NOT NULL,
	response TEXT NOT NULL,
	UNIQUE(start_date, end_date) ON CONFLICT REPLACE
);
