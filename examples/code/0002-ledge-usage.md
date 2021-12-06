---
title: ledge usage
created_date: 2021-12-02 20:06:20
updated_date: 2021-12-02 20:06:20
---

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="https://theme.ledge.ink/styles.css">
    <base href="/">
</head>
<body>
<div id="ledge-content"></div>
</body>
<script src="https://theme.ledge.ink/runtime-es5.js"></script>
<script src="https://theme.ledge.ink/polyfills-es5.js"></script>
<script src="https://theme.ledge.ink/main-es5.js"></script>
<script>
    var content = document.getElementById('ledge-content');
    var tile = document.createElement('ledge-theme');
    var text = `
\`\`\`table-step
| 项目 / 过程管理 | 配置管理 | 构建  | 测试 / 质量 | 制品 / 部署 | 基础设施 | 沟通协作 | 可视化   |
|---|----|---|---|----|----|----|----|
| Jira          | Gitee   | Maven | Junit      | Ubran code | VMWare  | 招呼     | Tableau |
| Tracker       | Rational ClearCase |  Gradle | Cucumber | Fit2Cloud | OpenShift | 移事通 | Grafana |
| VP            | CMDB | NPM | JMeter     | B9         | Cloud Foundry | | Kibana |
| Confluence    |   Firefly    | Ant   | RobotFramework | JFrog Artifactory | | |  Prometheus |
| ITIL          |    | MSBuild | Protractor | | | | ElasticSearch |
|               |           |  Docker  | Sonar | | | | X-Pack |
|               |           |        | BlackDuck | | | | |

config: {"rowHeight": "340px" ,"colors": [{"bg":"#e55852","font":"#b71a09"},{"bg":"#e98832","font":"#c85113"},{"bg":"#f0d668","font":"#b88d0f"},
{"bg":"#a4c9cf","font":"#598893"},{"bg":"#47c0af","font":"#175a54"},
{"bg":"#387fd5","font":"#9ac9f5"},{"bg":"#7753df","font":"#cbb5f8"},{"bg":"#485cde","font":"#a0b1f3"}]}
\`\`\`
`
    tile.setAttribute('content', text);

    content.appendChild(tile);
</script>
</html>
```