<template>
    <div>
        <h1>{{title}}</h1>
    </div>
</template>

<script>
    export default {
        name: "HeadPatternSelector",
        data() {
            return {
                title: '选择表头解析模式',
                headParser: null,

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
                this.headParser = this.setting.headParser;
                //
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: this.canProceed,
                    onSubmit: this.onSubmit,
                });
            },
            canProceed: function () {
                return this.headParser;
            },
            // beforeSubmit: null,
            onSubmit: function () {
                this.$store.dispatch('app/updateSettingValue', {headParser: this.headParser});
            },
            // afterSubmit: null,
        }
    }
</script>

<style scoped>

</style>