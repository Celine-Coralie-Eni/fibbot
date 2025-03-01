# Title

## Fibbot Project

## Description 

A GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. 

## Introduction 

 Begin by creating a new Rust project named fibbot using the command
 
 ```
 cargo new fibbot
```

Then do move to the fibbot directory and open the project in VScode using the ```code .``` command.

## Body

After this, create a new GitHub repository named fibbot and give it a public access and readme file. In this repository create an action.yml file and precise the neccesary actions to be performed and update your action.yml to include two parameters which are enable_fib and max_threshold and implement input parsing in your Rust code.

Create a workflow file in the .github/workflows file and update it to contain the following:

```

steps
   - name: Build and run action
        uses: ./
        with:
          enable_fib: true
          max_threshold: 100
          pr_number: ${{ github.event.pull_request.number }}

```

## Clone my Repository

```

git clone https://github.com/Celine-Coralie-Eni/fibbot

```

## NOTE!

Ensure your workflow has the read and write permissions .

Ensure you must have generated a GitHub token.

After to monitor your logs after every commit you make.

Create a dummy branch in your repository to test and ensure that everything is in place.






