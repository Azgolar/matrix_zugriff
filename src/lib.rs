pub mod matrix;
pub mod test;

pub mod bin {
    pub mod ein_d_matrix {
        pub mod basis_1d;
        pub mod basis_laenge_1d;
        pub mod split_at_1d;
        pub mod slice_1d;
        pub mod iterator_1d;
        pub mod unsicher_1d;
    }

    pub mod zwei_d_matrix {
        pub mod basis_2d;
        pub mod basis_laenge_2d;
        pub mod split_at_2d;
        pub mod slice_2d;
        pub mod iterator_2d;
        pub mod unsicher_2d;
    }
}

