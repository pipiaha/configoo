<template>
    <div>
        <h1>文件上传</h1>

        <div>
            <el-radio v-model="multiple" label="false">单文件</el-radio>
            <el-radio v-model="multiple" label="true">文件夹</el-radio>
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
    import decoder from "./modules/decoder";

    export default {
        name: "FileSelector",
        data() {
            return {
                multiple: false,
                options: {
                    // target: '/uploadCategory',//SpringBoot后台接收文件夹数据的接口
                    // testChunks: false//是否分片-不分片
                },
            };
        },
        methods: {
            onUploadConfirm: function (dir, file) {
                // console.log(file);
                if (file.isFolder) {
                    return;
                }
                decoder.decode(file, {});
            },
        }
    }
</script>

<style scoped>

</style>