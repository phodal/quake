---
title: transflow support http/https api and fetch next
author: 
status: Spike
priority: Low
created_date: 2021-12-16 00:14:27
updated_date: 2021-12-16 00:14:27
---

refs: [https://camel.apache.org/manual/rest-dsl.html](https://camel.apache.org/manual/rest-dsl.html)

```java
rest("/say/hello")
    .get().route().transform().constant("Hello World");
rest("/say/bye")
    .get().consumes("application/json").route().transform().constant("Bye World").endRest()
    .post().to("mock:update");
```

Samples:

```
from(
    rest('https://examples.com/api/blog').next()?
 )
.to(<quake-network>)
.map([])
```

some config:

1. next link: `.next(field('@'))`;
2. offsets: `.next(param('offset', 40))`;

and a fetch next API

```
@Event(): fetchSuccess={(data) => {}}
@Props(): url;
@Props(): type

load global quake config;

<fetch-api url="@prop" type="github">

</fetch-api>
```

