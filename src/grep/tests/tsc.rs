#[cfg(test)]
mod tests {
    use crate::grep::{grep, params::GrepParamsBuilder, tests::utils::assert_paths};

    const TSC_ERROR_MESSAGE: &str = r#"
❯ bun tsc --noEmit
src/domains/commu/components/TestCard.tsx:134:11 - error TS2322: Type 'boolean' is not assignable to type 'number'.

134           isRemovable={isRemovable}
              ~~~~~~~~~~~

  src/domains/commu/components/TestCard.tsx:22:3
    22   isRemovable: number;
         ~~~~~~~~~~~
    The expected type comes from property 'isRemovable' which is declared here on type 'IntrinsicAttributes & Readonly<{ displayName: string; profileImageUrl: string; badgeImageUrl: string | undefined; authorType: IrvineV1FeedEntryAuthorType; ... 7 more ...; onOpenReportDialog: () => void; }>'

src/domains/commu/components/TestCard.tsx:93:9 - error TS2322: Type 'number' is not assignable to type 'boolean'.

93         isRemovable={isRemovable}
           ~~~~~~~~~~~

  src/domains/commu/components/TestCardContainer.tsx:16:3
    16   isRemovable: boolean;
         ~~~~~~~~~~~
    The expected type comes from property 'isRemovable' which is declared here on type 'IntrinsicAttributes & Props'


Found 2 errors in 2 files.

Errors  Files
     1  src/domains/commu/components/TestListCard.tsx:134
     1  src/domains/commu/components/TestListCard.tsx:93
error: "tsc" exited with code 2
            "#;

    #[test]
    fn tsc_error_message() {
        let params = GrepParamsBuilder::new()
            .content(Some(TSC_ERROR_MESSAGE.to_string()))
            .build()
            .unwrap();

        assert_paths(
            grep(&params),
            vec![
                "src/domains/commu/components/TestCard.tsx",
                "src/domains/commu/components/TestCardContainer.tsx",
                "src/domains/commu/components/TestListCard.tsx",
            ],
        );
    }
}
