<template>
    <div>
        <div>xls/xlsx</div>
        <el-form label-position="left" label-width="140px">
            <el-form-item label="数据起始行">
                <el-input @change="onMetaParserChange" v-model="dataLine"/>
            </el-form-item>
            <el-divider/>
            <el-form-item label="服务器列名称所在行">
                <el-input @change="onMetaParserChange" v-model="server.name"/>
            </el-form-item>
            <el-form-item label="服务器列名称提取">
                <el-input @change="onMetaParserChange" v-model="server.namePattern"/>
            </el-form-item>
            <el-form-item label="服务器列类型所在行">
                <el-input @change="onMetaParserChange" v-model="server.type"/>
            </el-form-item>
            <el-form-item label="服务器列类型提取">
                <el-input @change="onMetaParserChange" v-model="server.typePattern"/>
            </el-form-item>
            <el-divider/>
            <el-form-item label="客户端列名称所在行">
                <el-input @change="onMetaParserChange" v-model="client.name"/>
            </el-form-item>
            <el-form-item label="客户端列名称提取">
                <el-input @change="onMetaParserChange" v-model="client.namePattern"/>
            </el-form-item>
            <el-form-item label="客户端列类型所在行">
                <el-input @change="onMetaParserChange" v-model="client.type"/>
            </el-form-item>
            <el-form-item label="客户端列类型提取">
                <el-input @change="onMetaParserChange" v-model="client.typePattern"/>
            </el-form-item>
        </el-form>
    </div>
</template>

<script>
    export default {
        name: "XlsHeader",
        model: {
            prop: 'value',
            event: 'metaParserChange'
        },
        props: {
            value: Object,
        },
        data() {
            return {
                dataLine: null,
                server: {
                    name: null,
                    type: null,
                    namePattern: null,
                    typePattern: null,
                },
                client: {
                    name: null,
                    type: null,
                    namePattern: null,
                    typePattern: null,
                },
            }
        },
        created() {
            if (this.value) {
                this.dataLine = this.value.dataLine;
                this.server = (this.value.server || {});
                this.client = (this.value.client || {});
            }
        },
        methods: {
            onMetaParserChange: function () {
                this.$emit('metaParserChange', {server: this.server, client: this.client, dataLine: this.dataLine});
            }
        }
    }
</script>

<style scoped>

</style>