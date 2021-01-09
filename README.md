# configoo

#### 介绍

游戏配置文件解析器

文件解析

转码csv sql json lua

内容+格式检查

### 需求

1. 解析转码 
    
```
    in
    读取文件夹，过滤文件
    读取单个文件
    文件类型支持： xlsx/xls/csv
    命名方式：文件名/sheet名 + 自定义前后缀
    
    parse
    表头行
    类型行/无类型
    前后类型分离

    out
    输出json
    输出sql
    输出csv

```


#### 软件架构
软件架构说明


#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

electron+vue 环境安装

* 工程目录下添加文件 `vue.config.js`
* `src`目录下添加`background.js`

* electron 版本选择了v11.1.1；
* vue-cli-plugin-electron-builder版本选择了2.0.0-rc.5；
* 多次使用npm安装均失败，效果奇差。这里选择yarn，并使用了代理fq，为WebStorm配置了Proxy；
* 执行 `yarn add electron` （没代理fq可以尝试[切换yarn源](https://zhuanlan.zhihu.com/p/108370177)）；
* 运行`electron`命令检查安装结果；
* 执行 `vue add electron-builder`；

参考

 [vue-cli-plugin-electron-builder](https://github.com/nklayman/vue-cli-plugin-electron-builder/tree/v2.0.0-rc.4)
 
 [electron](https://github.com/electron/electron)

#### 使用说明

1.  web调试
2.  web打包
3.  electron+vue调试

To start a development server:
If you use Yarn (strongly recommended):

```shell script
yarn electron:serve
```

or if you use NPM:

```shell script
npm run electron:serve
```

4.  electron+vue打包

To build your app:
With Yarn:

```shell script
yarn electron:build
```

or with NPM:

```shell script
npm run electron:build
```
To see more documentation, [visit our website](https://nklayman.github.io/vue-cli-plugin-electron-builder/guide/guide.html)

electron+vue 的调试和打包，还可以使用`package.json`中的命令：

```json
{
    "electron:build": "vue-cli-service electron:build",
    "electron:serve": "vue-cli-service electron:serve"
}
```

即：

```shell script 
    vue run electron:build
    vue run electron:serve
```

打包过程中，首次使用`electron-builder`有可能会卡在下载
`electron-xx-xx.zip` ,`winCodeSign-xxx.gz`, `nsis-xx.gz`,`nsis-resource-xx.gz`上，
手动下载后，放在

* windows 分别放在`C:\\user\AppData\electron\Cache\`下,
 `C:\\user\AppData\electron-builder\cache\winCodeSign\`下（需要解压）
 `C:\\user\AppData\electron-builder\cache\nsis\`下（需要解压）
 `C:\\user\AppData\electron-builder\cache\nsis-resources\`下（需要解压）
* MacOS `???`

[参考](https://blog.csdn.net/cctvcqupt/article/details/87904368)

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 码云特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  码云官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解码云上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是码云最有价值开源项目，是码云综合评定出的优秀开源项目
5.  码云官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  码云封面人物是一档用来展示码云会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
