---
title: knowledge graph for mapping
author: 
status: Spike
priority: Low
created_date: 2022-01-12 23:08:53
updated_date: 2022-01-12 23:08:53
---

通识：http://openconcepts.openkg.cn/


中文：http://openkg.cn/dataset/chinese-wordnet

### HowNet

HowNet: [https://github.com/thunlp/OpenHowNet](https://github.com/thunlp/OpenHowNet)

```python
import OpenHowNet
hownet_dict = OpenHowNet.HowNetDict()

>>> # Get all the senses represented by the word "苹果".
>>> result_list = hownet_dict.get_sense("苹果")
>>> print("The number of retrievals: ", len(result_list))
The number of retrievals:  8

>>> print("An example of retrievals: ", result_list)
An example of retrievals:  [No.244401|apple|苹果, No.244402|malus pumila|苹果, No.244403|orchard apple tree|苹果, No.244396|apple|苹果, No.244397|apple|苹果, No.244398|IPHONE|苹果, No.244399|apple|苹果, No.244400|iphone|苹果]
```


## Library

### Graph Database

[https://github.com/cayleygraph/cayley](https://github.com/cayleygraph/cayley)

### Rust

### Sophia

link: https://github.com/pchampin/sophia_rs

> A Rust toolkit for RDF and Linked Data.



#### Saga

https://github.com/victor-iyi/sage

> Sage is an open source Knowledge Graph used to represent linked-data. It comprises of varieties of features that makes it stand out amongst other (open source) Knowledge Graphs.


### TerminusDB

link: https://github.com/terminusdb/terminusdb

>  TerminusDB is an open source knowledge graph and document store. Use it to build versioned data products. 


