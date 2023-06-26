package cfgs

// import "redhare/game/cfg"


type RhythmMasterConfig全局配置 struct {
	// cfg.Cfg
    
    Id    int32 // 表的ID
    Excellent    float32 // 一级时间误差
    Perfect    float32 // 二级时间误差
    Good    float32 // 节拍最大误差时间
    BeatBegin    float32 // 节拍渐变时间
    Excellent2score    int32 // 完美分数
    Perfect2score    int32 // 不错分数
    Good2score    int32 // 正常分数
    Movetime    float32 // 预告移动时间
    Combos    []int32 // 连续不错/优秀/完美次数
    Settletime    float32 // 节拍渐变时间
}


func (c *RhythmMasterConfig全局配置) GetID() int32 {
    return c.Id
}

func (c *RhythmMasterConfig全局配置) SetID(id int32)  {
    c.Id = id
}