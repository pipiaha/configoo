package cfgs

// import "redhare/game/cfg"


type RhythmMasterConfig资源配置 struct {
	// cfg.Cfg
    
    Id    int32 // 资源序号
    Name    string // 资源名称
    SrcPath    string // 资源路径
}


func (c *RhythmMasterConfig资源配置) GetID() int32 {
    return c.Id
}

func (c *RhythmMasterConfig资源配置) SetID(id int32)  {
    c.Id = id
}