extern crate piecetable;
extern crate quickcheck;
extern crate rand;

use quickcheck::{quickcheck, Arbitrary, Gen};
use rand::Rng;

use piecetable::PieceTable;

// Note: These also implicitly test the iterator.

#[derive(Clone, Debug)]
struct InsertWithIndices<T: Arbitrary> {
    data: Vec<T>,
    indices: Vec<usize>,
}

impl<T: Arbitrary> Arbitrary for InsertWithIndices<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let mut indices: Vec<usize> = Vec::with_capacity(data.len());

        for i in (1..data.len()) {
            indices.push(g.gen_range(0, i));
        }

        InsertWithIndices {
            data: data,
            indices: indices,
        }
    }
}

#[derive(Clone, Debug)]
struct RemoveWithIndices<T: Arbitrary> {
    data: Vec<T>,
    indices: Vec<usize>,
}

impl<T: Arbitrary> Arbitrary for RemoveWithIndices<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let mut indices: Vec<usize> = Vec::with_capacity(data.len());

        if data.len() == 0 {
            return RemoveWithIndices { data: data, indices: vec![], };
        } else if data.len() == 1 {
            return RemoveWithIndices { data: data, indices: vec![0], };
        }

        for i in (1..g.gen_range(1, data.len())).rev() {
            indices.push(g.gen_range(0, i));
        }

        RemoveWithIndices {
            data: data,
            indices: indices,
        }
    }
}

#[test]
fn insert() {
    fn prop(recipe: InsertWithIndices<i32>) -> bool {
        let mut expected = Vec::with_capacity(recipe.data.len());
        let mut table = PieceTable::new(&[]);

        for (&i, &x) in recipe.indices.iter().zip(recipe.data.iter()) {
            expected.insert(i, x);
            table.insert(i, x);
        }

        expected.iter().collect::<Vec<&i32>>() ==
            table.iter().collect::<Vec<&i32>>()
    }

    quickcheck(prop as fn(InsertWithIndices<i32>) -> bool);
}

#[test]
fn remove() {
    fn prop(recipe: RemoveWithIndices<i32>) -> bool {
        let mut expected = recipe.data.clone();
        let mut table = PieceTable::new(&recipe.data);

        for &i in recipe.indices.iter() {
            expected.remove(i);
            table.remove(i);
        }

        expected.iter().collect::<Vec<&i32>>() ==
            table.iter().collect::<Vec<&i32>>()
    }

    quickcheck(prop as fn(RemoveWithIndices<i32>) -> bool);
}

#[test]
fn insert_and_remove() {
    fn prop(xs: Vec<i32>) -> bool {
        let mut expected = Vec::with_capacity(xs.len());
        let mut table = PieceTable::new(&[]);

        for (i, &x) in xs.iter().enumerate() {
            expected.insert(i / 2, x);
            table.insert(i / 2, x);

            if i % 2 == 0 {
                expected.remove(i / 3);
                table.remove(i / 3);
            }
        }

        expected.iter().collect::<Vec<&i32>>() ==
            table.iter().collect::<Vec<&i32>>()
    }

    quickcheck(prop as fn(Vec<i32>) -> bool);
}
