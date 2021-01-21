<template>
    <div>
        <h1>{{title}}</h1>
        <div>
            <el-radio v-model="selectedType" :label="'csv'" border>csv</el-radio>
            <el-radio v-model="selectedType" :label="'xls/xlsx'" border>xls/xlsx</el-radio>
            <el-radio v-model="selectedType" :label="'xml'" border>xml</el-radio>
        </div>
    </div>
</template>

<script>
    export default {
        name: "TypeSelector",
        data() {
            return {
                title: '选择文件类型',
                selectedType: null,
            };
        },
        created() {
            console.log('type selector created');
        },
        mounted() {
            console.log('type selector mounted');
        },
        updated() {
            console.log('type selector updated');
        },
    }
</script>

<style scoped>

</style>