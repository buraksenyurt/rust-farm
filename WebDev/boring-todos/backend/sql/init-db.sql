DROP DATABASE IF EXISTS task_force_db;
DROP USER IF EXISTS app_user;

CREATE USER app_user PASSWORD 'P@ssw0rd';
CREATE DATABASE task_force_db owner app_user ENCODING = 'UTF-8';