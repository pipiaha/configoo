<template>
    <div>
        <h1>选择文件类型</h1>
        <div>
            <el-radio v-model="selectedType" :label="'csv'" border>csv</el-radio>
            <el-radio v-model="selectedType" :label="'xls/xlsx'" border>xls/xlsx</el-radio>
            <el-radio v-model="selectedType" :label="'xml'" border>xml</el-radio>
        </div>
    </div>
</template>

<script>
    export default {
        name: "Home",
        data() {
            return {
                selectedType: null,
            };
        }
    }
</script>

<style scoped>

</style>