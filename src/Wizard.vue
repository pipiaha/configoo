<template>
    <el-card class="wizard-wrapper">
        <div slot="header" class="card-header clearfix">
            <span class="title">configoo - {{currentModule.title}}</span>
            <!--                <el-button style="float: right; padding: 3px 0" type="text">操作按钮</el-button>-->
        </div>
        <div class="wizard-body">
            <transition>
                <router-view/>
            </transition>
        </div>
        <div class="card-footer wizard-footer">
            <el-button type="primary" size="small" @click="prev">上一步</el-button>
            <el-button type="primary" size="small" @click="next">下一步</el-button>
        </div>
    </el-card>
</template>

<script>
    import decoder from "./modules/decoder";

    export default {
        name: "Wizard",
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

            let modIndex = this.mods.findIndex(m => m.path === this.$route.path);
            if (modIndex >= 0) {
                let mod = this.mods[modIndex];
                this.$router.push(mod.path).then(() => {
                    this.$store.dispatch('app/updateModuleIndex', modIndex);
                }).catch(() => {
                    // 解决相同route 路由报错问题
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

<style scoped>
    .wizard-wrapper {
        /*min-height: 500px;*/
    }

    .wizard-wrapper .wizard-body {
        position: relative;
        min-height: 500px;
    }

    .wizard-wrapper .wizard-footer {
        float: right;
        margin-right: 16px;
    }
</style>