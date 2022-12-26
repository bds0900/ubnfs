create table if not exists Users(
    id serial primary key,
    name varchar(255) not null
);

create table if not exists Matchs(
    id serial primary key,
    user1 serial,
    user2 serial,

    constraint fk_user1 foreign key (user1) references Users (id),
    constraint fk_user2 foreign key (user2) references Users (id)
);

create table if not exists Likes(
    id serial primary key,
    sender serial,
    reciver serial,

    constraint fk_sender foreign key (sender) references Users (id),
    constraint fk_reciver foreign key (reciver) references Users (id)
)
