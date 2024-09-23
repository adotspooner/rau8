# Rust Always UTF-8 (rau8)

Dealing with UTF-8 encoded strings in C can be challenging. C lacks built-in guarantees that strings are valid UTF-8, leading to potential errors or vulnerabilities when working with text data. Rust, on the other hand, natively uses UTF-8 for strings and ensures that all strings are valid UTF-8 at all times, providing a safer and more reliable way to handle text.

The **Rust Always UTF-8 (rau8)** library was written in Rust and designed to be used within C programs. By leveraging Rust's guarantees around string encoding, this library allows you to seamlessly integrate Rust's UTF-8 handling capabilities into your C codebase, ensuring that your strings remain valid and minimizing the risk of encoding-related issues.

## Why Rust for UTF-8?

* Built-in UTF-8 validation: Rust strings are always valid UTF-8, reducing the likelihood of runtime errors related to malformed strings.
* Seamless C integration: This library provides a straightforward interface to bring Rust's robust UTF-8 string handling into your C applications.

## Contributions

Contributions to Rust Always UTF-8 (rau8) are welcome and encouraged! Whether you're fixing bugs, improving documentation, or adding new features, your help is greatly appreciated. Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License, a permissive open-source license that allows you to freely use, modify, and distribute the code. For more details, please see the LICENSE file in the repository.