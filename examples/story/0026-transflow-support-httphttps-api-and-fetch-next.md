---
title: transflow support http/https api and fetch next
author: 
status: Spike
priority: Low
created_date: 2021-12-16 00:14:27
updated_date: 2021-12-16 00:14:27
---

Samples:

```
from('https://examples.com/api/blog').to(<quake-network>);
```

and a fetch next API

```
<fetch-next url="@prop" fetchSuccess={(data) => {}}>
</fetch-next>
```

