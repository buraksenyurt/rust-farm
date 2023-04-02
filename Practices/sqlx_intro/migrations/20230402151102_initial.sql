create table categories(
    id bigserial primary key,
    title varchar(50) unique not null
);

insert into categories (title) values ('Kitap');
insert into categories (title) values ('Dergi');
insert into categories (title) values ('Plak');

create table products(
    id bigserial primary key,
    category_id bigint not null,
    title varchar(100) not null,
    unit_price real default 0.00,
    constraint fk_category foreign key (category_id) references categories(id) on delete cascade
);
