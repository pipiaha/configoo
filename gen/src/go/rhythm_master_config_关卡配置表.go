package cfgs

// import "redhare/game/cfg"


type RhythmMasterConfig关卡配置表 struct {
	// cfg.Cfg
    
    Id    int32 // 关卡序号
    Name    string // 关卡名称
    Score2star    []int32 // 星星所需积分
    Bgpath    string // 背景图片名称
    Audioclips    []int32 // 使用的音乐片段
    Limit2reward    int32 // 每日刷金币次数
    Map    int32 // 交互区域模式(按钮个数)
    Star2coin    []int32 // 星级对应金币
    Playmode    int32 // 模式
    IconPaths    []int32 // 按钮图标
    PreIconPaths    []int32 // 按钮预告图标
    Frequency    int32 // 按钮交换频率
    Globalmode    int32 // 全局配置
}


func (c *RhythmMasterConfig关卡配置表) GetID() int32 {
    return c.Id
}

func (c *RhythmMasterConfig关卡配置表) SetID(id int32)  {
    c.Id = id
}