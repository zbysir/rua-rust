## 在 Docker 上运行测试用 NSQ
> MacOS

nsqd

```
docker run --name nsqd -p 4150:4150 -p 4151:4151 -d nsqio/nsq /nsqd --broadcast-address=127.0.0.1 --lookupd-tcp-address=host.docker.internal:4160
```

lookupd
```
docker run --name lookupd -p 4160:4160 -p 4161:4161 -d nsqio/nsq /nsqlookupd
```

nsqadmin
```
docker run -d --name nsqadmin -p 4171:4171 nsqio/nsq /nsqadmin --lookupd-http-address=host.docker.internal:4161
```