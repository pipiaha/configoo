<template>
    <div>
        <div>
            <el-radio v-model="multiple" :label="false">单文件</el-radio>
            <el-radio v-model="multiple" :label="true">文件夹</el-radio>
        </div>
        <div v-if="multiple">
            <uploader :key="1" :options="options" class="uploader-example"
                      @file-success="onUploadConfirm">
                <uploader-unsupport></uploader-unsupport>
                <uploader-drop>
                    <uploader-btn :directory="true" :single="true">选择文件夹</uploader-btn>
                </uploader-drop>
                <uploader-list></uploader-list>
            </uploader>
        </div>
        <div v-else> 选择文件</div>
    </div>
</template>

<script>
    import decoder from "../../modules/decoder";

    export default {
        name: "FileSelector",
        data() {
            return {
                title: '文件上传',
                multiple: false,
                options: {
                    // target: '/uploadCategory',//SpringBoot后台接收文件夹数据的接口
                    // testChunks: false//是否分片-不分片
                },
            };
        },
        computed: {
            setting() {
                return this.$store.state.app.setting;
            },
        },
        created() {
            console.log('file selector created');
            this.onLoad();
        },
        mounted() {
            console.log('file selector mounted');
        },
        methods: {
            onUploadConfirm: function (dir, file) {
                // console.log(file);
                if (file.isFolder) {
                    return;
                }
                decoder.decode(file, {});
            },
            onLoad: function () {
                this.$store.dispatch('app/updateModuleValue', {
                    title: this.title,
                    canProceed: () => true,
                });
            },
        }
    }
</script>

<style scoped>

</style>