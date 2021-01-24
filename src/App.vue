<template>
    <div id="app">
        <img alt="Vue logo" src="./assets/logo.png">
        <!--        <input id="b" type="file" webkitdirectory>-->
        <uploader :key="1" :options="options" class="uploader-example"
                  @file-success="onUploadConfirm">
            <uploader-unsupport></uploader-unsupport>
            <uploader-drop>
                <uploader-btn :directory="true" :single="true">选择文件夹</uploader-btn>
            </uploader-drop>
            <uploader-list></uploader-list>
        </uploader>

        <el-card>
            <div slot="header" class="card-header clearfix">
                <span class="title">configoo - {{currentModule.title}}</span>
                <!--                <el-button style="float: right; padding: 3px 0" type="text">操作按钮</el-button>-->
            </div>
            <div>
                <transition>
                    <router-view/>
                </transition>
            </div>
            <div class="card-footer">
                <el-button type="primary" size="small" @click="prev">上一步</el-button>
                <el-button type="primary" size="small" @click="next">下一步</el-button>
            </div>
        </el-card>

        <HelloWorld msg="Welcome to Your Vue.js App"/>
    </div>
</template>

<script>
    import HelloWorld from './components/HelloWorld.vue'
    import decoder from './modules/decoder';

    export default {
        name: 'App',
        components: {
            HelloWorld
        },
        data() {
            return {
                options: {
                    // target: '/uploadCategory',//SpringBoot后台接收文件夹数据的接口
                    // testChunks: false//是否分片-不分片
                },

            };
        },
        computed: {
            mods() {
                return this.$store.state.app.mods;
            },
            currentModule() {
                return this.$store.state.app.currentModule;
            },
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
        },
        mounted() {
            // FIXME 相同route 路由报错问题
            let modIndex = this.mods.findIndex(m => m.path === this.$route.path);
            if (modIndex >= 0) {
                let mod = this.mods[modIndex];
                this.$router.push(mod.path).then(() => {
                    this.$store.dispatch('app/updateModuleIndex', modIndex);
                }).catch(() => {
                    this.$store.dispatch('app/updateModuleIndex', modIndex);
                });
            }
        },
        methods: {
            onUploadConfirm: function (dir, file) {
                // console.log(file);
                if (file.isFolder) {
                    return;
                }
                decoder.decode(file, {});
            },
            canProceed: function () {
                return this.currentModule.canProceed && this.currentModule.canProceed();
            },
            prev: function () {
                let mod = this.mods[this.currentModule.index - 1];
                if (!mod) {
                    this.$message.error('没有啦！');
                    return;
                }
                this.$router.push(mod.path).then(() => {
                    this.$store.dispatch('app/updateModuleIndex', this.currentModule.index - 1)
                        .catch(err => err);
                });
            },
            next: function () {
                let mod = this.mods[this.currentModule.index + 1];
                if (!mod) {
                    this.$message.error('没有啦！');
                    return;
                }
                if (!this.canProceed()) {
                    this.$message.error('当前步骤未完成');
                    return;
                }
                //
                if (this.currentModule.beforeSubmit) {
                    this.currentModule.beforeSubmit();
                }
                if (this.currentModule.onSubmit) {
                    this.currentModule.onSubmit();
                }
                if (this.currentModule.afterSubmit) {
                    this.currentModule.afterSubmit();
                }
                console.log(this.currentModule);
                this.$router.push(mod.path).then(() => {
                    this.$store.dispatch('app/updateModuleIndex', this.currentModule.index + 1)
                        .catch(err => err);
                });
            }
        }
    }
</script>

<style>
    #app {
        font-family: Avenir, Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
        color: #2c3e50;
        margin-top: 60px;
    }

    .card-header {
        margin: 12px 0;
        min-height: 20px;
    }

    .card-header .title {
        float: left;
        font-weight: bold;
        font-size: 16px;
    }

    .card-footer {
        margin: 12px 0;
    }
</style>
