import XLSX from "xlsx";

const decode = function (file, options) {
    let type = file.name.substring(file.name.lastIndexOf('.') + 1);
    let fun = types[type];
    if (!fun) {
        console.log('decoder not found for ' + type);
        return;
    }
    fun.apply(file, options);
}

const decodeXlsx = function (file, options) {
    let headerRow = options.headerRow;
    console.log(headerRow)
    const fileReader = new FileReader();
    fileReader.onload = (ev) => {
        try {
            const data = ev.target.result;
            const workbook = XLSX.read(data, {
                type: 'binary'
            })
            const wsname = workbook.SheetNames[0]// 取第一张表
            const ws = XLSX.utils.sheet_to_json(workbook.Sheets[wsname])// 生成json表格内容
            console.log(ws, 'ws是表格里的数据，且是json格式')
            // this.tabData = ws
            // console.log(this.tabData, 'tabdata')
            // 重写数据
            // this.$refs.upload.value = ''
        } catch (e) {
            return false
        }
    };
    fileReader.readAsBinaryString(file.file);
}

const decodeJson = function (file, options) {
    console.log(options)
}

const types = {
    'json': decodeXlsx,
    'xlsx': decodeJson,
}

export default {
    types,
    decode
}