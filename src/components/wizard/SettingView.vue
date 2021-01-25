<template>
    <div>
        <el-table :data="rows" stripe border>
            <el-table-column
                    prop="key"
                    label="key">
            </el-table-column>
            <el-table-column
                    prop="value"
                    label="value">
            </el-table-column>
        </el-table>
    </div>
</template>

<script>
    export default {
        name: "SettingView",
        data() {
            return {
                title: 'setting',
                rows: [],
            }
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            this.onLoad();
        },
        mounted() {
        },
        methods: {
            onLoad: function () {
                //
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: () => true,
                }).then(() => {
                    this.rows = this.settingRows();
                });
            },
            settingRows: function () {
                return Object.keys(this.setting).map(k => {
                    return {key: k, value: this.setting[k]};
                });
            }
        }
    }
</script>

<style scoped>

</style>