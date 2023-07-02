# Contributing to ddd-rust

Thank you for your interest in contributing to the ddd-rust project! We welcome any contributions that help improve the library and enhance its functionality. Please take a moment to review the guidelines outlined below before getting started.

## Issue Reporting

If you encounter any bugs, have questions, or would like to suggest new features, please open an issue in the issue tracker. Provide a clear and detailed explanation of the problem or feature request, including any relevant code snippets or error messages.

## Pull Requests

We encourage you to submit pull requests with improvements, bug fixes, or new features. To contribute code, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bug fix.
   - Branch names should be descriptive and follow the format `feature/{issue-id}-{name-of-issue}`.
   - For example, if the issue ID is 42 and the issue is about implementing a new command, the branch name could be `feature/42-implement-new-command`.
2. Ensure that your code adheres to the Rust programming style guidelines:
   - Use 2 spaces for indentation.
   - Write clean, readable code with meaningful variable and function names.
3. Write tests to cover your code changes and ensure that existing tests pass.
4. Update the documentation as necessary to reflect your changes.
5. Make sure your code passes the existing linting and formatting checks.
6. Commit your changes with a clear message that describes the purpose and details of your contribution.
   - Include the issue ID at the end of the commit message in the format `(#{issue-id})`.
   - For example, a commit message could be `Implement new command (#{issue-id})`.
7. Submit a pull request with a clear title and description explaining the purpose and details of your contribution.
8. Be responsive to any feedback or change requests during the review process.

## Domain-Driven Design (DDD)

ddd-rust follows the principles of Domain-Driven Design (DDD) to build well-structured, maintainable, and expressive domain models. When contributing, please keep the following guidelines in mind:

1. Aim for a clear separation of concerns and maintain a rich domain model.
2. Follow the established concepts and patterns used within the library for aggregates, events, commands, and value objects.
3. Write code that aligns with the ubiquitous language of the domain, using meaningful and domain-specific terminology.
4. Ensure that your contributions are consistent with the overall design philosophy and architecture of ddd-rust.

## Code of Conduct

Please note that by participating in this project, you are expected to adhere to the [Code of Conduct](CODE_OF_CONDUCT.md). Please be respectful and considerate of others when interacting with the community.

We appreciate your contributions and look forward to working with you to make ddd-rust even better!
