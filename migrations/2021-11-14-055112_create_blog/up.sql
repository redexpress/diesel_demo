CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE blogs (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL references users(id),          -- foreign key 1
    title VARCHAR NOT NULL
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    blog_id INT NOT NULL,
    body VARCHAR NOT NULL,
    constraint fk_blog_id foreign key(blog_id) references blogs(id)  -- foreign key 2
);

-- 修改表添加外键约束
-- alter table 表名 add constraint 外键别名 foreign key(列名) references 父表(列名);
-- 删除外键约束
-- alter table 表名 drop constraint 外键别名;

CREATE TABLE animals (
    id SERIAL PRIMARY KEY,
    species VARCHAR NOT NULL,
    legs INT NOT NULL,
    name VARCHAR
);


