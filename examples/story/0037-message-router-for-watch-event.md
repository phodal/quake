---
title: message router for watch event
author: 
status: Spike
priority: Low
created_date: 2021-12-31 19:46:45
updated_date: 2021-12-31 19:46:45
---



## Rule Engine


Drools

## Enterprise Integration Pattern

### Apache Camel

如开始所述，Apache Camel是一个集成框架。camel可以做到:

- 路由:将数据有效负载(也称为“消息”)从源系统发送到目标系统
- 中介:消息处理，如基于一个或多个消息属性过滤消息、修改消息的某些字段、通过API调用进行充实等。


> Apache Camel中关于Endpoint最直白的解释就是，Camel作为系统集成的基础服务组件，在已经编排好的路由规则中，和其它系统进行通信的设定点。这个“其它系统”，可以是存在于本地或者远程的文件系统，可以是进行业务处理的订单系统，可以是消息队列服务，可以是提供了访问地址、访问ip、访问路径的任何服务。Apache Camel利用自身提供的广泛的通信协议支持，使这里的“通信”动作可以采用大多数已知的协议，例如各种RPC协议、JMS协议、FTP协议、HTTP协议。

#### 路由和端点

Route 是最基本的构造，我们用它来定义消息从源移动到目的地时应该采取的路径。我们使用领域特定语言(DSL)定义路由。

- 在Camel上下文中加载路由，并在触发路由时用于执行路由逻辑。每条路由都由Camel上下文中的唯一标识符标识。
- 端点表示消息的源和目的地。它们通常通过它们的uri在领域特定语言(DSL)中被引用。端点的例子可以是web应用程序的URL或消息传递系统的源或目的地。


### Spring Integration

refs: [https://www.tony-bro.com/posts/1578338213/index.html](https://www.tony-bro.com/posts/1578338213/index.html)

> 从源头Enterprise Integration Pattern来说，它认为企业内部各个子服务基于消息集成，在这种方式下各个组件之间的交互将不再使用远程调用等同步方式，而是通过向目标子系统发送一个消息来令该子系统执行某个功能，在消息成功发送之后，调用方即可以开始对其它任务进行处理，而不再是同步调用过程中的等待。在使用这种处理方式时，一个系统的吞吐量可以大大增加。这种应用场景被抽象为Pipes-and-Filters模型：

![Spring Integration](https://www.tony-bro.com/posts/1578338213/pipe-filter.png)

在Spring Integration中，pipe和filter加上消息本身构成了三大基本组件：

1. Message：即消息本身，它由Payload和Header两部分组成，Payload是对任意Java对象的包装而Header则包含了消息的元数据信息，同时header也常用于Http、Mail等其他消息头部的转换。其基接口为Message<T>，需要注意的是通用的消息实现是不可变的。

2. Message Channel：即Pipes-and-Filters模型中的pipe，它是消息传输的载体，通常可以分为point-to-point（点对点）和publish-subscribe（发布订阅）两种行为模式。此外从通道是否保存消息的角度来说，通道还分为Pollable Channel和Subscribable Channel两种。
  - Pollable Channel：保存消息，消费者需要主动拉取消息，核心接口为PollableChannel。
   - Subscribable Channel：可订阅型通道，不存储消息，消费者被动通知消息，核心接口为SubscribableChannel。
这种划分方式也是API接口的划分方式，不同的通道类型对消息流程的处理会有不同的表现形式。

3. Message Endpoint：即Pipes-and-Filters模型中的Filter，它是消息的消费端，通常与外部系统对接。Spring Integration提供了多种不同的EndPoint满足不同的需求。

DSL samples:

```java
Configuration
@EnableIntegration
public class MyConfiguration {

    @Bean
    public AtomicInteger integerSource() {
        return new AtomicInteger();
    }

    @Bean
    public IntegrationFlow myFlow() {
        return IntegrationFlows.from(integerSource::getAndIncrement,
                                         c -> c.poller(Pollers.fixedRate(100)))
                    .channel("inputChannel")
                    .filter((Integer p) -> p > 0)
                    .transform(Object::toString)
                    .channel(MessageChannels.queue())
                    .get();
    }
}
```
