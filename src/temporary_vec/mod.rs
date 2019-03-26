use std::ops::Deref;

/// Vectorに対して一時的に操作を行い、使い終わったら最初のVectorに戻す。
/// ## sample
/// ```
/// fn main() {
///     let mut v = vec![0, 1];
///     let values = vec![100, 200, 300, 0, 1, 10, 11, 4];
///     display(&v);
/// 
///     let temp = TemporaryVec::onto(&mut v, values);
///     display(&temp.to_vec());
///     display(&*temp);
///     temp.revert();
/// 
///     display(&v);
/// }
/// 
/// fn display<T>(disp: &[T]) where T: std::fmt::Debug {
///     println!("{:?}", disp);
/// }
/// ```
/// ## output
/// ```
/// [0, 1]
/// [0, 1, 100, 200, 300, 0, 1, 10, 11, 4]
/// [0, 1, 100, 200, 300, 0, 1, 10, 11, 4]
/// [0, 1]
/// ```
#[derive(Debug)]
pub struct TemporaryVec<'a, T> {
    inner: &'a mut Vec<T>,
    len: usize,
}

impl<'a, T> TemporaryVec<'a, T> {
    /// 渡されたvectorに値をセットして自分自身を返す。
    /// パラメータのvaluesはmoveされる。
    pub fn onto(target: &'a mut Vec<T>, values: Vec<T>) -> TemporaryVec<'a, T> {
        let len = values.len();
        values.into_iter().for_each(|v| target.push(v));
        
        TemporaryVec { 
            inner: target,
            len: len,
        }
    }

    /// 渡されたvectorに値をセットして自分自身を返す。
    /// パラメータのvaluesはCloneしてセットされる。
    pub fn onto_from_clone(target: &'a mut Vec<T>, values: &[T]) -> TemporaryVec<'a, T> where T: std::clone::Clone {
        let len = values.len();
        values.iter().cloned().for_each(|v| target.push(v));
        
        TemporaryVec { 
            inner: target,
            len: len,
        }
    }

    /// 内部で保持するVectorの参照を返す。
    pub fn as_vec(&self) -> &Vec<T> {
        &self.inner
    }

    /// 最初に渡されたVectorのborrowを解放し追加した値も全てキャンセルされる。
    pub fn revert(self) {}
}

impl<'a, T> Drop for TemporaryVec<'a, T> {
    /// dropされた時に最初に渡されたvectorを元に戻す。
    fn drop(&mut self) {
        for _i in 0..self.len {
            self.inner.pop();
        }
    }
}

impl<'a, T> Deref for TemporaryVec<'a, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.inner
    }
}

