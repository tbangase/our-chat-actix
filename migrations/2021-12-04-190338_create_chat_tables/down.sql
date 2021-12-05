-- This file should undo anything in `up.sql`
SET FOREIGN_KEY_CHECKS = 0;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS subscribes;
SET FOREIGN_KEY_CHECKS = 1;

