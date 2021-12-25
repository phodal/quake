---
title: "biz visual analysis"
created_date: 2021-12-25 15:16:05
updated_date: 2021-12-25 17:33:06
tag: 
author: 
---

> 字段构成问题，是问题分析的基础；记录描述业务，是数据聚合的基础。

## GitHub related projects

### Superset

GitHub link: <https://github.com/apache/superset>

Apache Superset是一个现代的、可供企业使用的商业智能（BI）网络应用。它具有快速、轻量、直观和选择性加载的特性。正因为这些特性，不管是简单的饼图还是非常复杂的deck.gl空间图表，不同技术的开发者都能容易地探索和可视化他们的数据。

 ![Samples](https://github.com/apache/superset/raw/master/superset-frontend/src/assets/images/screenshots/gallery.jpg)

Superset被设计为高可用的云原生应用。它被设计来扩展大规模分布式环境，而且在容器内运行效果非常好。虽然你可以轻松地在适当的设置或简单地在您的笔记本上测试启动Superset，但是它对扩展平台却几乎没有限制。Superset作为云原生应用的另一重含义是它很灵活，可以让您自己选择：

* Web服务器（Gunicorn、Nginx、Apache）
* 元数据数据库引擎（MySQL、Postgres、MariaDB等）
* 消息队列（Redis、RabbitMQ、SQS等）
* Results Backend（S3、Redis、Memcached等）
* 缓存层（Memcached、Redis等）

### Redash

GitHub link: <https://github.com/getredash/redash>

Redash 的设计目的是让任何人，无论技术水平高低，都能驾驭大大小小的数据力量。SQL 用户利用 Redash 来探索、查询、可视化和共享来自任何数据源的数据。他们的工作反过来使他们组织中的任何人都可以使用这些数据。每天，世界各地数千个组织的数百万用户使用Redash来开发洞察力和做出数据驱动的决策。

**特性**

* 基于浏览器：浏览器中的所有内容，均带有可共享的URL。
* 易于使用：无需掌握复杂软件即可立即获得数据。
* 查询编辑器：使用架构浏览器快速组成SQL和NoSQL查询并自动完成。
* 可视化和仪表板：通过拖放创建漂亮的可视化文件，并将它们组合成一个仪表板。
* 共享：通过共享可视化及其相关查询，可以轻松进行协作，从而可以对报告和查询进行同行评审。
* 计划刷新：按您定义的定期自动更新图表和仪表盘。
* 警报：定义条件并在数据更改时立即收到警报。
* REST API：可以通过REST API使用UI进行的所有操作。
* 对数据源的广泛支持：可扩展​​的数据源API，具有对一长串常见数据库和平台的本机支持。

### Metabase

GitHub link: <https://github.com/metabase/metabase>

**Metabase** 是一款易用、开源、技术成熟、不断并快速迭代的报表系统。 使用**metabase**可以省去很多前后端的开发工作，只需要进行数据清洗计算转存等相关开发。 在目前无开发人力的情况下，这是较为完美的BI系统解决方案。

**特性**

* 设置仅需 5 分钟
* 让团队中的成员在不知道 SQL 的情况下提出问题
* 丰富美丽的仪表板与自动刷新和全屏模式
* 分析师和数据专家专属 SQL 模式
* 为你的团队创建规范细分和指标以供使用
* 发送数据到 Slack 或电子邮件与 Pulses 的日程安排
* 使用 Metabot 随时查看 Slack 中的数据
* 通过重命名、注释和隐藏字段为你的团队人性化数据

 ![Screenshot](https://github.com/metabase/metabase/raw/master/docs/metabase-product-screenshot.png)




