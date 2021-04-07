#[derive(Debug)]
pub struct IdGeneratorOptions {
    /// 雪花计算方法,（1-漂移算法|2-传统算法），默认1
    pub method: u8,
    /// 基础时间（ms单位），不能超过当前系统时间
    pub base_time: i64,
    /// 机器码，与 WorkerIdBitLength 有关系
    pub worker_id: u32,
    /// 机器码位长，范围：1-21（要求：序列数位长+机器码位长不超过22）
    pub worker_id_bit_length: u8,
    /// 序列数位长，范围：2-21（要求：序列数位长+机器码位长不超过22）
    pub seq_bit_length: u8,
    /// 最大序列数（含），（由 SeqBitLength 计算的最大值）
    pub max_seq_number: u32,
    /// 最小序列数（含），默认5，不小于5，不大于 MaxSeqNumber
    pub min_seq_number: u32,
    /// 最大漂移次数（含），默认2000，推荐范围 500-20000（与计算能力有关）
    pub top_over_cost_count: u32,
}

impl IdGeneratorOptions {
    pub fn new(worker_id: u32) -> IdGeneratorOptions {
        return IdGeneratorOptions {
            method: 1,
            worker_id,
            base_time: 1582136402000,
            worker_id_bit_length: 6,
            seq_bit_length: 12,
            max_seq_number: 0,
            min_seq_number: 5,
            top_over_cost_count: 100000,
        };
    }
}