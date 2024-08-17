pub mod params;

use params::GrepParams;
use regex::Regex;

/// Extract path in string message with regex
pub fn grep(params: GrepParams) -> Vec<String> {
    // Define the regex pattern to match file paths
    let re = Regex::new(r"(\S+/[\w\-/\.]+(?:\:\d+(?:\:\d+)?)?)").unwrap();
    // Create an empty vector to hold the matches
    let mut paths = Vec::new();

    // Iterate over all matches in the message
    for cap in re.captures_iter(&params.content) {
        // Push the matched path to the vector
        paths.push(cap[1].to_string());
    }

    if !params.lines {
        paths = paths
            .iter()
            .map(|path| {
                let parts: Vec<&str> = path.split(':').collect();
                parts[0].to_string()
            })
            .collect::<Vec<String>>();
    }

    paths.sort();
    if params.unique {
        paths.dedup();
    }

    paths
}

#[cfg(test)]
mod tests {
    use params::GrepParamsBuilder;
    use pretty_assertions::assert_eq;

    use super::*;

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
    fn extract_path_test() {
        let params = GrepParamsBuilder::new()
            .content("hello world /home/username /etc/passwd".to_string())
            .build()
            .unwrap();

        assert_eq!(grep(params), ["/etc/passwd", "/home/username"]);
    }

    #[test]
    fn extract_path_from_tsc_error_message() {
        let params = GrepParamsBuilder::new()
            .content(TSC_ERROR_MESSAGE.to_string())
            .build()
            .unwrap();

        assert_eq!(
            grep(params),
            [
                "src/domains/commu/components/TestCard.tsx",
                "src/domains/commu/components/TestCardContainer.tsx",
                "src/domains/commu/components/TestListCard.tsx",
            ]
        );
    }

    #[test]
    fn extract_path_from_error_message() {
        let params = GrepParamsBuilder::new()
            .content(r#"error: "tsc" exited with code 2"#.to_string())
            .build()
            .unwrap();

        assert_eq!(grep(params), Vec::<String>::new());
    }

    #[test]
    fn extract_path_without_lines_from_tsc_error_message() {
        let params = GrepParamsBuilder::new()
            .content(TSC_ERROR_MESSAGE.to_string())
            .lines(Some(true))
            .build()
            .unwrap();

        assert_eq!(
            grep(params),
            [
                "src/domains/commu/components/TestCard.tsx:134:11",
                "src/domains/commu/components/TestCard.tsx:22:3",
                "src/domains/commu/components/TestCard.tsx:93:9",
                "src/domains/commu/components/TestCardContainer.tsx:16:3",
                "src/domains/commu/components/TestListCard.tsx:134",
                "src/domains/commu/components/TestListCard.tsx:93",
            ]
        );
    }
}
