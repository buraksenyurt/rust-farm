create table categories(
    id bigserial primary key,
    name varchar(50) unique not null
);

create table products(
    id bigserial primary key,
    category_id bigint not null,
    title varchar(100) not null,
    unit_price decimal(10,2) default 0.00,
    constraint fk_category foreign key (category_id) references categories(id) on delete cascade
);
