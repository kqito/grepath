#[cfg(test)]
mod tests {
    use crate::grep::{grep, params::GrepParamsBuilder, tests::utils::assert_paths};

    const ESLINT_ERROR_MESSAGE: &str = r#"
/src/hogehoge/hogefuga/apple/pine.tsx
  90:9  error  Dangerous property 'dangerouslySetInnerHTML' found  react/no-danger

/src/hogehoge/hogefuga/banana/grape.tsx
  4:8  error  'default' import from 'next/image' is restricted.

/src/hogehoge/hogefuga/carrot/orange.test.ts
  2:10  warning  'renderToastToNotification' is defined but never used  @typescript-eslint/no-unused-vars

/src/domains/animal/hooks/usePostAnimalCanFollow.ts
  23:50  error  Forbidden non-null assertion  @typescript-eslint/no-non-null-assertion

/src/domains/animal/hogefuga/dog/cat.tsx
  102:7  warning  Using `<img>` could result in slower LCP and higher bandwidth. Consider using `<Image />` from `next/image` to automatically optimize images. This may incur additional usage or cost from your provider. See: https://nextjs.org/docs/messages/no-img-element  @next/next/no-img-element

/src/domains/animal/hogefuga/elephant/lion.tsx
  84:13  warning  Using `<img>` could result in slower LCP and higher bandwidth. Consider using `<Image />` from `next/image` to automatically optimize images. This may incur additional usage or cost from your provider. See: https://nextjs.org/docs/messages/no-img-element  @next/next/no-img-element

/src/domains/animal/hogefuga/fish/tiger.tsx
  91:6  warning  React Hook useEffect has a missing dependency: 'reset'. Either include it or remove the dependency array  react-hooks/exhaustive-deps

/src/domains/animal/hogefuga/fox/goatContainer.tsx
  88:6  warning  React Hook useEffect has missing dependencies: 'connect' and 'firebaseUser'. Either include them or remove the dependency array  react-hooks/exhaustive-deps

/src/domains/animal/hogefuga/giraffe/hippo.tsx
  57:6  warning  React Hook useEffect has missing dependencies: 'authSessionStorage?.subscription?.fallbackUrl', 'authSessionStorage?.subscription?.successUrl', 'locale', 'push', 'setSessionStorage', and 't'. Either include them or remove the dependency array  react-hooks/exhaustive-deps

/src/domains/plant/hogefuga/ivy/jasmine.tsx
  40:6  warning  React Hook useEffect has missing dependencies: 'isPrevLiked' and 'startAnimation'. Either include them or remove the dependency array  react-hooks/exhaustive-deps

/src/domains/home/hogefuga/kale/lemon.tsx
  28:5  error  Unexpected console statement  no-console
  30:5  error  Unexpected console statement  no-console
            "#;

    #[test]
    fn eslint_error_message() {
        let params = GrepParamsBuilder::new()
            .content(Some(ESLINT_ERROR_MESSAGE.to_string()))
            .no_validate(Some(true))
            .build()
            .unwrap();

        assert_paths(
            grep(&params),
            vec![
                "nextjs.org",
                "src/domains/animal/hogefuga/dog/cat.tsx",
                "src/domains/animal/hogefuga/elephant/lion.tsx",
                "src/domains/animal/hogefuga/fish/tiger.tsx",
                "src/domains/animal/hogefuga/fox/goatContainer.tsx",
                "src/domains/animal/hogefuga/giraffe/hippo.tsx",
                "src/domains/animal/hooks/usePostAnimalCanFollow.ts",
                "src/domains/home/hogefuga/kale/lemon.tsx",
                "src/domains/plant/hogefuga/ivy/jasmine.tsx",
                "src/hogehoge/hogefuga/apple/pine.tsx",
                "src/hogehoge/hogefuga/banana/grape.tsx",
                "src/hogehoge/hogefuga/carrot/orange.test",
            ],
        );
    }
}
