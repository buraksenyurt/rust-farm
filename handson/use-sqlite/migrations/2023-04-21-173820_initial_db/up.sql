-- Your SQL goes here
create table if not exists categories(
    id integer primary key autoincrement,
    title text not null
);

insert into categories (title) values ("Platform");
insert into categories (title) values ("Action-Adventure");
insert into categories (title) values ("Puzzle");
insert into categories (title) values ("Role-Playing");
insert into categories (title) values ("Simulation");
insert into categories (title) values ("Strategy");
insert into categories (title) values ("Sports");

create table if not exists games(
    id integer primary key autoincrement,
    category_id integer not null,
    title text not null,
    start integer not null,
    foreign key (category_id) references categories (id)
);