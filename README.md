# diesel_demo

本项目主要演示使用Postgres数据库时diesel的配置

## 使用步骤
安装 diesel
```
cargo install diesel_cli --no-default-features --features postgres
```
如果安装出错，需要安装Postgres的链接库后再试
Debian/Ubuntu 安装方法
```
sudo apt install libpq-dev
```
RHEL/CentOS 安装方法
```
sudo yum install postgresql-devel
```

使用 diesel 参考：http://diesel.rs/guides/getting-started

