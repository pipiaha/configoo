<template>
    <div>
        <h1>{{title}}</h1>
        <div v-if="setting.selectedType==='xls/xlsx'">
            <XlsHeader v-model="metaParser"/>
        </div>
    </div>
</template>

<script>
    import XlsHeader from "../header/XlsHeader";

    export default {
        name: "HeadPatternSelector",
        components: {XlsHeader},
        data() {
            return {
                title: '选择表头解析模式',
                metaParser: {},

                options: {
                    'xls/xlsx': [],
                    'json': [],
                    'xml': []
                }
            };
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            console.log('HeadPatternSelector created');
            this.onLoad();
        },
        mounted() {
            console.log('HeadPatternSelector mounted');
        },
        methods: {
            onLoad: function () {
                //
                this.metaParser = this.setting.metaParser;
                //
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: this.canProceed,
                    onSubmit: this.onSubmit,
                });
            },
            canProceed: function () {
                return this.metaParser;
            },
            // beforeSubmit: null,
            onSubmit: function () {
                this.$store.dispatch('app/updateSettingValue', {metaParser: this.metaParser});
            },
            // afterSubmit: null,
        }
    }
</script>

<style scoped>

</style>