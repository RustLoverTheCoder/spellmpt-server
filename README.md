# server

## docker-compose

- 构建yml中某个服务的镜像
    ```sh
    docker-compose build
    ```
- 查看已经启动的服务状态
    ```sh
    docker-compose ps
    ```
- 停止某个服务
    ```sh
    docker-compose kill
    ```
- 可以查看某个服务的log
    ```sh
    docker-compose logs
    ```
- 打印绑定的public port
    ```sh
    docker-compose port
    ```
- pull服务镜像
    ```sh
    docker-compose pull
    ```
- 启动yml定义的所有服务
    ```sh
    docker-compose up
    ```
- 停止yml中定义的所有服务
    ```sh
    docker-compose stop
    ```
- 启动被停止的yml中的所有服务
    ```sh
    docker-compose start
    ```
- 强行停止yml中定义的所有服务
    ```sh
    docker-compose kill
    ```
- 删除yml中定义的所有服务
    ```sh
    docker-compose rm
    ```
- 重启yml中定义的所有服务
    ```sh
    docker-compose restart
    ```
- 扩展某个服务的个数，可以向上或向下
    ```sh
    docker-compose scale
    ```