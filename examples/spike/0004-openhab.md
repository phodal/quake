---
title: "openHab spike"
author: 
done_by: 
created_date: 2022-03-15 09:37:11
updated_date: 2022-03-15 10:07:44
---

$$wiki$$: 


## Concepts

### Things, Channels, Bindings, Items and Links

Things 是可以物理添加到系统中的实体。事物可能提供不止一种功能（例如，Z-Wave 多传感器可以提供运动检测器并测量室温）。事物不必是物理设备；它们还可以代表 Web 服务或任何其他可管理的信息和功能来源。

Thingss 通过 Channel 暴露其能力。是否安装利用 Channel 反映的特定功能，取决于它是否已配置为这样做。配置系统时，您不必使用事物提供的所有功能。您可以通过查看 Thing 的 Binding 文档来了解 Thing 可用的 Channels。

Binding 可以被认为是软件适配器，使您的家庭自动化系统可以使用事物。它们是附加组件，提供了一种将项目链接到物理设备的方法。它们还抽象出该设备的特定通信要求，以便框架可以更通用地处理它。

Items 表示应用程序可以在用户界面或自动化逻辑中使用的功能。项目有一个状态，它们可以接收命令。

Things 和 Items 之间的粘合剂是 Links。 Links 是一个 Channel 和一个 Item 之间的关联。如果 Channel 链接到 Item，则它是“启用的”，这意味着 Item 表示的功能可通过该 Channel 访问。频道可以链接到多个项目，项目可以链接到多个频道。

示例图：

 ![Things to Channel](https://www.openhab.org/assets/img/thing-devices-1.bd432b36.png)

## DSL

### Widgets DSL

Custom Widgets: <https://www.openhab.org/docs/tutorial/custom_widgets.html>

```javascript
uid: widget_5c7a60b74f
props:
  parameterGroups: []
  parameters:
    - name: prop1
      label: Prop 1
      type: TEXT
      description: A text prop
    - name: item
      label: Item
      type: TEXT
      context: item
      description: An item to control
tags: []
component: f7-card
config:
  title: '=(props.item) ? "State of " + props.item : "Set props to test!"'
  footer: =props.prop1
  content: =items[props.item].displayState || items[props.item].state
```

### Sitemap DSL

```javascript
sitemap demo label="My home automation" {
    Frame label="Date" {
        Text item=Date
    }
    Frame label="Demo" {
        Switch item=Lights icon="light"
        Text item=LR_Temperature label="Livingroom [%.1f °C]"
        Group item=Heating
        Text item=LR_Multimedia_Summary label="Multimedia [%s]" icon="video" {
            Selection item=LR_TV_Channel mappings=[0="off", 1="DasErste", 2="BBC One", 3="Cartoon Network"]
            Slider item=LR_TV_Volume
        }
    }
}
```

### Rule DSL 

```javascript
rule "Start wake up light on sunrise"
when
    Channel "astro:sun:home:rise#event" triggered
then
    switch(receivedEvent.getEvent()) {
        case "START": {
            Light.sendCommand(OFF)
        }
    }
end
```

结合语义化的 Channel 设计 + Rule DSL，可以用于配置自动化的控制设备。



