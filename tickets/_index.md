# Brownfield Recon Ticket Index

Build sequencing lives in [_build_order.md](_build_order.md).

Status: all tickets in this recon set are done.

| id | title | lens | severity | effort | blast_radius | depends_on |
| --- | --- | --- | --- | --- | --- | --- |
| REL-001 | stop counting directories as files in stats breakdown | reliability | S2 | S | 1 module, 1 command | [] |
| REL-002 | return nonzero for unsupported checksum algorithms | reliability | S2 | S | 1 module, 1 command, 1 MCP tool | [] |
| REL-003 | reject invalid fixed-value CLI options | reliability | S2 | S | 2 modules, 2 commands | [] |
| REL-005 | fail invalid duplicate size filters | reliability | S2 | S | 1 module, 1 command | [] |
| ARCH-001 | route tree traversal through the shared walker contract | architecture | S2 | M | 1 module, 2 commands, 1 MCP tool | [] |
| REL-004 | normalize search extension filters | reliability | S3 | S | 1 module, 1 command, 1 MCP tool | [] |
| MAINT-001 | add CI coverage for the demo project | maintainability | S3 | S | CI plus demo package | [] |
| ARCH-002 | unify MCP tool schema and argument builders | architecture | S3 | M | 1 module, 12 MCP tools | [] |
| REL-006 | classify symlink paths in info | reliability | S3 | M | 1 module, 1 command | [] |
| SEC-001 | harden file-backed clipboard storage | security | S3 | M | 1 module, 1 command | [] |
