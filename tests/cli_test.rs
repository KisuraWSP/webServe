#[cfg(test)]
mod cli_tests{
    #[test]
    fn check_options(){
        let option = vec![1,2,3,4,5];
        assert_eq!(option[0],1);
        assert_eq!(option[1],2);
        assert_eq!(option[2],3);
        assert_eq!(option[3],4);
        assert_eq!(option[4],5);
    }
}