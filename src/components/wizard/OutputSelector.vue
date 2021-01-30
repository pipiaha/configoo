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

            <el-form-item label="输出内容">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :xs="24" :sm="24" :md="4">
                        <el-select v-model="output.records.type" placeholder="选择输出格式">
                            <el-option v-for="(option,index) in typeOptions" :key="index" :label="option"
                                       :value="option"/>
                        </el-select>
                    </el-col>
                    <el-col :xs="24" :sm="24" :md="12">
                        <el-input placeholder="选择输出路径" v-model="output.records.path"/>
                    </el-col>
                </el-row>
            </el-form-item>

            <el-divider/>

            <el-form-item label="输出源文件">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :xs="24" :sm="24" :md="4">
                        <el-select v-model="output.sources.type" placeholder="选择输出格式">
                            <el-option v-for="(option,index) in srcOptions" :key="index" :label="option"
                                       :value="option"/>
                        </el-select>
                    </el-col>
                    <el-col :xs="24" :sm="24" :md="12">
                        <el-input placeholder="选择输出路径" v-model="output.sources.path"/>
                    </el-col>
                </el-row>
            </el-form-item>
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
                        type: null,
                        path: null,
                    },
                    sources: {
                        type: null,
                        path: null,
                    }
                },
                typeOptions: [
                    'csv', 'json', 'sql', 'lua'
                ],
                srcOptions: [
                    '.java', '.cs',
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