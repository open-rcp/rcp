#!/bin/bash
# Workflow validator script for RCP project
# Usage: ./validate_workflows.sh

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

WORKFLOW_DIR="$(dirname "$0")/.."
ERRORS=0
WARNINGS=0

echo -e "${YELLOW}Validating GitHub Actions workflow files...${NC}"

# Check for syntax errors in yaml files
for file in "$WORKFLOW_DIR"/*.yml; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    echo -n "Checking $filename... "
    
    # Check YAML syntax
    if python3 -c "import yaml; yaml.safe_load(open('$file'))" 2>/dev/null; then
      echo -e "${GREEN}✓ YAML syntax OK${NC}"
    else
      echo -e "${RED}✗ YAML syntax error${NC}"
      ERRORS=$((ERRORS + 1))
    fi
    
    # Check for hardcoded secrets
    if grep -q "GITHUB_TOKEN: " "$file"; then
      echo -e "${RED}✗ Hardcoded GITHUB_TOKEN found in $filename${NC}"
      ERRORS=$((ERRORS + 1))
    fi
    
    # Check for latest action versions
    if grep -q "actions/checkout@v3" "$file"; then
      echo -e "${YELLOW}⚠ Using older version of actions/checkout. Consider upgrading to v4${NC}"
      WARNINGS=$((WARNINGS + 1))
    fi
    
    if grep -q "actions/upload-artifact@v3" "$file"; then
      echo -e "${YELLOW}⚠ Using older version of actions/upload-artifact. Consider upgrading to v4${NC}"
      WARNINGS=$((WARNINGS + 1))
    fi
  fi
done

# Summary
echo ""
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
  echo -e "${GREEN}All workflows passed validation!${NC}"
  exit 0
elif [ $ERRORS -eq 0 ]; then
  echo -e "${YELLOW}Validation completed with $WARNINGS warnings.${NC}"
  exit 0
else
  echo -e "${RED}Validation failed with $ERRORS errors and $WARNINGS warnings.${NC}"
  exit 1
fi
