use ac_library::Monoid;

fn ceil_pow2(n: u32) -> u32 {
    32 - n.saturating_sub(1).leading_zeros()
}

impl<M: Monoid> Default for DisjointSparseTable<M> {
    fn default() -> Self {
        DisjointSparseTable::new(0)
    }
}
impl<M: Monoid> DisjointSparseTable<M> {
    pub fn new(n: usize) -> DisjointSparseTable<M> {
        vec![M::identity(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::S>> for DisjointSparseTable<M> {
    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let log = ceil_pow2(n as u32) as usize;
        let table = vec![vec![M::identity();1<<log];log];
        let fact = vec![0;n+1];
        let mut st = DisjointSparseTable {n,log, fact,table,  };
        st.init(&v);
        st
    }
}
impl<M: Monoid> DisjointSparseTable<M> {
    fn init(&mut self,v:&Vec<M::S>){
        for i in 0..self.n {
            self.table[self.log-1][i] = v[i].clone();
        }
        for i in (0..self.log-1).rev() {
            let sep=1<<(self.log-i-1);
            for j in (1..usize::MAX).step_by(2){
                if sep*j>=self.n{
                    break;
                }
                let start=sep*j;
                let mut left=M::identity();
                for k in (start-sep..start).rev(){
                    left = M::binary_operation(&left, &self.table[self.log-1][k]);
                    self.table[i][k] = left.clone();
                }
                let mut right=M::identity();
                for k in start..start+sep {
                    right = M::binary_operation(&right, &self.table[self.log-1][k]);
                    self.table[i][k] = right.clone();
                }
            }
        }
    }
}

pub struct DisjointSparseTable<M>
where
    M:Monoid,
{
    pub n:usize,
    pub log:usize,
    pub fact:Vec<usize>,
    pub table:Vec<Vec<M::S>>,
}