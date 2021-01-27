<template>
    <div>
        <h1>{{title}}</h1>
        <div>
            <el-radio v-for="(option,index) in typeOptions" :key="index" v-model="selectedType" :label="option" border>
                {{option}}
            </el-radio>
        </div>
    </div>
</template>

<script>
    export default {
        name: "TypeSelector",
        data() {
            return {
                title: '选择文件类型',
                typeOptions: [
                    'csv', 'xls/xlsx', 'xml'
                ],
                selectedType: null,
            };
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            console.log('type selector created');
            this.onLoad();
        },
        mounted() {
            console.log('type selector mounted');
        },
        methods: {
            onLoad: function () {
                //
                this.selectedType = this.setting.selectedType;
                //
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: this.canProceed,
                    onSubmit: this.onSubmit,
                });
            },
            canProceed: function () {
                return this.selectedType;
            },
            // beforeSubmit: null,
            onSubmit: function () {
                this.$store.dispatch('app/updateSettingValue', {selectedType: this.selectedType});
            },
            // afterSubmit: null,
        }
    }
</script>

<style scoped>

</style>