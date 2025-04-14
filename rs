#!/bin/bash
rsync --update --exclude 'rs' --exclude '.git' -v -a nimn.eu nimneu:
