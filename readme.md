# Rotate HTTP Proxy

Sometime when working with chrome you can only specify only one http proxy. But for many purpose you need some sort of rotating upstream proxy. There are many situation you have many http proxies but you need to rotate effectively.

So this project gives one example how you can make upstream http proxy that can rotate ips. There are many other way to do such stuff.

Currently tokio runtime is used but in future I will make it not to depend on async runtime.

# Use

Either you can download binary from release tab or build manually.

Here instead of config file i preferred Env variable which seems easier and portable too.

```
PROXY_PATH=/Users/quantum/Desktop/code/http_proxy/proxies.txt  LISTEN_ADDR=127.0.0.1:8100 cargo run
```

After that u can test if its really working or not by using curl

```
curl -x 127.0.0.1:8100 https://jsonip.com
```

By default you may not specify LISTEN_ADDR. If you dont specify LISTEN_ADDR then it will default to 127.0.0.1:8100

### Help

If you need help then you can either use Issue tab or contact me via email given below

Shirshak Bajgain (shirshak55 at pm.me)
