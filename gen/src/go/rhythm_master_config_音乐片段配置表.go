package cfgs

// import "redhare/game/cfg"


type RhythmMasterConfig音乐片段配置表 struct {
	// cfg.Cfg
    
    Id    int32 // 音乐片段序号
    Name    string // 音乐片段名称
    Beats    []int32 // 节拍时间(第几秒出现节拍)
    PosArray    []string // 亮灯位置
    SrcPath    string // 资源路径
    TimeOffset    float32 // 时间偏移
    Speed    float32 // 播放速度
    MI_icon_ID    int32 // 乐器
    Foreshow    float32 // 下落时间
}


func (c *RhythmMasterConfig音乐片段配置表) GetID() int32 {
    return c.Id
}

func (c *RhythmMasterConfig音乐片段配置表) SetID(id int32)  {
    c.Id = id
}