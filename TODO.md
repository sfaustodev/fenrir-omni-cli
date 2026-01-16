# FENRIR Feature Implementation TODO

## Progress Bar Visualization
- [x] Add indicatif progress bar with big green rotating spinner to batch_executor.rs
- [x] Integrate progress bar into execute_sequential, execute_parallel, execute_pipeline methods
- [x] Update progress for each tool execution

## Target Setting Flow Modifications
- [x] Modify main.rs to suggest next steps after target is set
- [x] Add suggestions like "Run reconnaissance", "Start vulnerability scan", etc.

## Daemon Service Creation
- [x] Create new daemon.rs module for continuous scanning
- [x] Implement automatic network device, IoT, and app vulnerability scanning
- [x] Add daemon mode to CLI commands

## Async Conversion Verification
- [x] Verify all KaliTool execution methods are async tokio functions
- [x] Update any remaining synchronous methods in kali_tools_comprehensive.rs

## Security Breach Detection Command
- [x] Implement "security breach detected" terminal command
- [x] Add breach details display in CLI and main.rs
- [x] Integrate with existing BreachDetector

## CLI Updates
- [x] Add daemon mode support to cli.rs
- [x] Add progress visualization commands
- [x] Update bpaf parser for new commands

## Testing and Verification
- [x] Test progress bars in batch execution
- [x] Test daemon service functionality
- [x] Verify breach detection triggers terminal alerts
- [x] Ensure all async conversions work properly
