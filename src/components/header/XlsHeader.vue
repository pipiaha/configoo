<template>
    <div>
        <div>xls/xlsx</div>
        <el-form label-position="left" label-width="140px">
            <el-form-item label="数据起始行">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :span="4">
                        <el-input-number @change="onMetaParserChange" type="number" v-model="dataLine" :min="1"/>
                    </el-col>
                </el-row>
            </el-form-item>
            <el-divider/>
            <el-form-item label="服务器列名称">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :span="4">
                        <el-input-number :min="1" @change="onMetaParserChange" type="number" placeholder="所在行"
                                         v-model="server.name"/>
                    </el-col>
                    <el-col :span="12">
                        <el-input @change="onMetaParserChange" placeholder="提取" v-model="server.namePattern"/>
                    </el-col>
                </el-row>
            </el-form-item>
            <el-form-item label="服务器列类型">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :span="4">
                        <el-input-number :min="1" @change="onMetaParserChange" type="number" placeholder="所在行"
                                         v-model="server.type"/>
                    </el-col>
                    <el-col :span="12">
                        <el-input @change="onMetaParserChange" placeholder="提取" v-model="server.typePattern"/>
                    </el-col>
                </el-row>
            </el-form-item>
            <el-divider/>
            <el-form-item label="客户端列名称">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :span="4">
                        <el-input-number :min="1" @change="onMetaParserChange" type="number" placeholder="所在行"
                                         v-model="client.name"/>
                    </el-col>
                    <el-col :span="12">
                        <el-input @change="onMetaParserChange" placeholder="提取"
                                  v-model="client.namePattern"/>
                    </el-col>
                </el-row>
            </el-form-item>
            <el-form-item label="客户端列类型">
                <el-row :gutter="10" style="margin-bottom: 8px">
                    <el-col :span="4">
                        <el-input-number :min="1" @change="onMetaParserChange" type="number" placeholder="所在行"
                                         v-model="client.type"/>
                    </el-col>
                    <el-col :span="12">
                        <el-input @change="onMetaParserChange" placeholder="提取" v-model="client.typePattern"/>
                    </el-col>
                </el-row>

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