<template>
    <div>
        <el-form label-position="left" label-width="140px">
            <el-form-item label="命名方式">
                <el-select v-model="output.naming" placeholder="选择命名方式" style="float: left;padding-left: 5px">
                    <el-option label="文件名称" :value="'fileName'"/>
                    <el-option :disabled="setting.selectedType !=='xls/xlsx'" label="工作表名称"
                               :value="'sheetName'"/>
                </el-select>
            </el-form-item>

            <el-divider/>

            <div>
                <el-checkbox v-model="output.records.enable" label="输出内容" border style="margin-bottom: 8px"/>
                <el-form-item label="输出内容" v-if="output.records.enable">
                    <el-row :gutter="10" style="margin-bottom: 8px">
                        <el-col :xs="24" :sm="24" :md="4">
                            <el-select v-model="output.records.type" placeholder="选择输出格式">
                                <el-option v-for="(option,index) in typeOptions" :key="index" :label="option.name"
                                           :value="option.name"/>
                            </el-select>
                        </el-col>
                        <el-col :xs="24" :sm="24" :md="8">
                            <el-input placeholder="选择输出路径" v-model="output.records.path"/>
                        </el-col>
                        <el-col :xs="24" :sm="24" :md="10">
                            <el-checkbox v-model="output.records.useCustomTemplate" style="float: left;padding-left: 5px">自定义</el-checkbox>
                            <el-select v-if="!output.records.useCustomTemplate" v-model="output.records.template"
                                       placeholder="选择模板"
                                       style="">
                                <el-option v-for="(option,index) in typeOptions" :key="index" :label="option.template"
                                           :value="option.template"/>
                            </el-select>
                            <el-input v-else placeholder="自定义模板文件" v-model="output.records.template"/>
                        </el-col>
                    </el-row>
                </el-form-item>
            </div>
            <el-divider/>
            <div>
                <el-checkbox v-model="output.sources.enable" label="输出源文件" border style="margin-bottom: 8px"/>
                <el-form-item label="输出源文件" v-if="output.sources.enable">
                    <el-row :gutter="10" style="margin-bottom: 8px">
                        <el-col :xs="24" :sm="24" :md="4">
                            <el-select v-model="output.sources.type" placeholder="选择输出格式">
                                <el-option v-for="(option,index) in srcOptions" :key="index" :label="option.name"
                                           :value="option.name"/>
                            </el-select>
                        </el-col>
                        <el-col :xs="24" :sm="24" :md="6">
                            <el-input placeholder="选择输出路径" v-model="output.sources.path"/>
                        </el-col>
                        <el-col :xs="24" :sm="24" :md="6">
                            <el-select v-if="!output.records.useCustomTemplate" v-model="output.records.sources"
                                       placeholder="选择模板"
                                       style="float: left;padding-left: 5px">
                                <el-option v-for="(option,index) in srcOptions" :key="index" :label="option.template"
                                           :value="option.template"/>
                            </el-select>
                        </el-col>
                    </el-row>
                </el-form-item>
            </div>
        </el-form>
    </div>
</template>

<script>

    export default {
        name: "OutputSelector",
        data() {
            return {
                title: '输出设置',
                output: {
                    naming: null,
                    records: {
                        enable: false,
                        type: null,
                        path: null,
                        template: null,
                        useCustomTemplate: false,
                    },
                    sources: {
                        enable: false,
                        type: null,
                        path: null,
                        template: null,
                        useCustomTemplate: false,
                    }
                },
                typeOptions: [
                    {
                        name: 'csv',
                        template: 'csv_template',
                    },
                    {
                        name: 'json',
                        template: 'json_template',
                    },
                    {
                        name: 'sql',
                        template: 'sql_template',
                    },
                    {
                        name: 'lua',
                        template: 'lua_template',
                    },
                ],
                srcOptions: [
                    {
                        name: '.java',
                        template: 'java_src_template',
                    },
                    {
                        name: '.cs',
                        template: 'csharp_src_template',
                    },
                ],
            };
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            console.log('OutputSelector created');
            this.onLoad();
        },
        mounted() {
            console.log('OutputSelector mounted');
        },
        methods: {
            onLoad: function () {
                //
                this.output = this.setting.output;
                //
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: this.canProceed,
                    onSubmit: this.onSubmit,
                });
            },
            canProceed: function () {
                return this.output;
            },
            // beforeSubmit: null,
            onSubmit: function () {
                this.$store.dispatch('app/updateSettingValue', {output: this.output});
            },
            // afterSubmit: null,
        }
    }
</script>

<style scoped>

</style>